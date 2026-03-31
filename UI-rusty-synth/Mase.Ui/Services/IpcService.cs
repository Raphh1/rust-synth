using System.Diagnostics;
using System.Text.Json;
using System.Text.Json.Nodes;
using System.Threading.Tasks;

namespace Mase.Ui.Services;

public class IpcService
{
    private bool _isBackendConnected = false;

    public void Connect()
    {
        // TODO: lancer le process Rust et récupérer stdin/stdout
        _isBackendConnected = true;
    }

    public async Task<bool> SendCommandAsync(string commandType, string requestId, object? payload = null)
    {
        if (!_isBackendConnected) return false;

        var msg = new JsonObject
        {
            ["type"]      = commandType,
            ["requestId"] = requestId
        };

        if (payload != null)
        {
            var payloadNode = JsonSerializer.SerializeToNode(payload);
            if (payloadNode is JsonObject payloadObj)
                foreach (var kv in payloadObj)
                    msg[kv.Key] = kv.Value?.DeepClone();
        }

        var json = msg.ToJsonString();

        await Task.Delay(100);

        // TODO: écrire json + "\n" sur _backendProcess.StandardInput
        Debug.WriteLine($"[IPC OUT] {json}");

        return true;
    }
}
