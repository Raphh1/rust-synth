using System;
using System.Collections.Concurrent;
using System.Diagnostics;
using System.IO;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Threading;
using System.Threading.Tasks;

namespace Mase.Ui.Services;

public class IpcService : IDisposable
{
    private Process?  _process;
    private StreamWriter? _stdin;
    private bool _connected = false;

    // Chaque requête en attente de réponse : requestId → TaskCompletionSource
    private readonly ConcurrentDictionary<string, TaskCompletionSource<bool>> _pending = new();

    public void Connect()
    {
        var backendPath = FindBackendExecutable();
        if (backendPath == null)
        {
            Debug.WriteLine("[IPC] Backend executable not found.");
            return;
        }

        _process = new Process
        {
            StartInfo = new ProcessStartInfo
            {
                FileName               = backendPath,
                UseShellExecute        = false,
                RedirectStandardInput  = true,
                RedirectStandardOutput = true,
                RedirectStandardError  = true,
                CreateNoWindow         = true,
            }
        };

        _process.Start();
        _stdin = _process.StandardInput;
        _connected = true;

        // Thread de lecture des réponses en arrière-plan
        var readerThread = new Thread(ReadLoop) { IsBackground = true };
        readerThread.Start();

        // Log stderr du backend
        var errorThread = new Thread(() => {
            while (!_process.StandardError.EndOfStream)
                Debug.WriteLine($"[backend] {_process.StandardError.ReadLine()}");
        }) { IsBackground = true };
        errorThread.Start();

        Debug.WriteLine("[IPC] Backend connected.");
    }

    public async Task<bool> SendCommandAsync(string commandType, string requestId, object? payload = null)
    {
        if (!_connected || _stdin == null) return false;

        var msg = new JsonObject
        {
              ["type"]       = commandType,
              ["request_id"] = requestId
        };
        if (payload != null)
        {
            var options = new JsonSerializerOptions { PropertyNamingPolicy = JsonNamingPolicy.CamelCase };
            var payloadNode = JsonSerializer.SerializeToNode(payload, options);
            if (payloadNode is JsonObject payloadObj)
                foreach (var kv in payloadObj)
                    msg[kv.Key] = kv.Value?.DeepClone();
        }

        var json = msg.ToJsonString();
        Debug.WriteLine($"[IPC OUT] {json}");

        // Enregistrer la requête avant d'envoyer (évite race condition)
        var tcs = new TaskCompletionSource<bool>(TaskCreationOptions.RunContinuationsAsynchronously);
        _pending[requestId] = tcs;

        try
        {
            await _stdin.WriteLineAsync(json);
            await _stdin.FlushAsync();
        }
        catch (Exception ex)
        {
            _pending.TryRemove(requestId, out _);
            Debug.WriteLine($"[IPC] Write error: {ex.Message}");
            return false;
        }

        // Attendre la réponse avec timeout 2s
        var timeout = Task.Delay(2000);
        var completed = await Task.WhenAny(tcs.Task, timeout);
        _pending.TryRemove(requestId, out _);

        if (completed == timeout)
        {
            Debug.WriteLine($"[IPC] Timeout for requestId={requestId}");
            return false;
        }

        return await tcs.Task;
    }

    private void ReadLoop()
    {
        var stdout = _process!.StandardOutput;
        while (!stdout.EndOfStream)
        {
            var line = stdout.ReadLine();
            if (line == null) break;

            Debug.WriteLine($"[IPC IN] {line}");

            try
            {
                var doc = JsonDocument.Parse(line);
                var root = doc.RootElement;

                if (!root.TryGetProperty("request_id", out var idProp)) continue;
                var requestId = idProp.GetString() ?? "";

                if (!_pending.TryGetValue(requestId, out var tcs)) continue;

                var type = root.TryGetProperty("type", out var typeProp)
                    ? typeProp.GetString() : null;

                tcs.TrySetResult(type == "ok");
            }
            catch (Exception ex)
            {
                Debug.WriteLine($"[IPC] Parse error: {ex.Message}");
            }
        }

        Debug.WriteLine("[IPC] Backend process ended.");
        _connected = false;
    }

    /// Cherche l'exécutable backend selon l'OS et les chemins connus.
    private static string? FindBackendExecutable()
    {
        var baseDir = AppContext.BaseDirectory;
        var repoRoot = Path.GetFullPath(Path.Combine(baseDir, "..", "..", "..", ".."));

        var candidates = new[]
        {
            Path.Combine(baseDir, "backend.exe"),
            Path.Combine(repoRoot, "backend", "target", "release", "backend.exe"),
            Path.Combine(repoRoot, "backend", "target", "debug", "backend.exe"),
        };

        foreach (var path in candidates)
            if (File.Exists(path))
                return path;

        return null;
    }

    public void Dispose()
    {
        _connected = false;
        try { _stdin?.Close(); } catch { }
        try { _process?.Kill(); } catch { }
        _process?.Dispose();
    }
}
