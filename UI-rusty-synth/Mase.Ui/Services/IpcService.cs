using System;
using System.Diagnostics;
using System.IO;
using System.Text.Json;
using System.Threading.Tasks;

namespace Mase.Ui.Services;

/// <summary>
/// Service basique pour initier et gérer la communication IPC au format NDJSON.
/// </summary>
public class IpcService
{
    // Simulations pour l'instant (tant que le backend n'est pas branché).
    // Idéalement on garde une référence à Process et à son StandardInput/StandardOutput.
    // private Process? _backendProcess;
    
    // Simuler le fait que le backend est actif
    private bool _isBackendConnected = false;

    public void Connect()
    {
        // TODO: Lancer le `.exe` Rust et récupérer les flux stdin/stdout.
        // ProcessStartInfo startInfo = new ProcessStartInfo { ... };
        // _backendProcess = Process.Start(startInfo);
        _isBackendConnected = true;
    }

    public async Task<bool> SendCommandAsync(string commandType, string requestId, object? payload = null)
    {
        if (!_isBackendConnected) return false;

        // Construire le message IPC (NDJSON) selon le contrat MVP
        var message = new
        {
            type = commandType,
            requestId = requestId,
            // d'autres champs de payload pourraient être ajoutés en sérialisant un objet plus complexe
        };

        var json = JsonSerializer.Serialize(message);
        
        // Simuler un délai réseau/processing IPC
        await Task.Delay(100);

        // TODO: Ecrire le `json + "\n"` (NDJSON) sur _backendProcess.StandardInput
        Debug.WriteLine($"[IPC OUT] {json}");

        // Simuler un succès
        return true;
    }
}
