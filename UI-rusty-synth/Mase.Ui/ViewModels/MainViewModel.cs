using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using System;
using System.Threading.Tasks;
using Mase.Ui.Services;

namespace Mase.Ui.ViewModels;
public partial class MainViewModel : ViewModelBase
{
    private readonly IpcService _ipcService;

    public MainViewModel()
    {
        _ipcService = new IpcService();
        _ipcService.Connect();
    }

    [ObservableProperty]
    private string _windowTitle = "Rusty Synth - Main Rack";

    [ObservableProperty]
    private string _statusMessage = "Engine: Ready";

    [ObservableProperty]
    private string _playButtonText = "Play ▶";

    [ObservableProperty]
    private string _playButtonColor = "#4CAF50";
    private bool _isPlaying = false;

    [ObservableProperty]
    private string _engineStateText = "Stopped";

    [ObservableProperty]
    private string _engineStateColor = "#de0013ff"; 

    [ObservableProperty]
    private double _cutoff = 1000.0;

    [ObservableProperty]
    private string _cutoffDisplay = "Cutoff: 1000 Hz";

    partial void OnCutoffChanged(double value)
{
    CutoffDisplay = $"{value:F0} Hz";
}

    [RelayCommand]
    private void OpenPianoRoll()
    {
        StatusMessage = "Opening Piano Roll...";
    }

    [RelayCommand]
    private async Task PlayAsync()
    {
        if (_isPlaying) return;
        
        try
        {
            StatusMessage = "Starting playback...";
            
            // Generer un ID de requête basique
            string requestId = Guid.NewGuid().ToString();
            
            // Communication avec le backend Rust via le service IPC
            bool success = await _ipcService.SendCommandAsync("Play", requestId);

            if (success)
            {
                _isPlaying = true;
                PlayButtonText = "Playing ▶▶";
                PlayButtonColor = "#2E7D32";
                EngineStateText = "Playing";
                EngineStateColor = "#4CAF50";
                StatusMessage = "Engine: Playing";
            }
            else
            {
                StatusMessage = "Error: Failed to communicate with engine.";
            }
        }
        catch (Exception ex)
        {
            StatusMessage = $"Error: {ex.Message}";
        }
    }

    [RelayCommand]
    private async Task StopAsync()
    {
        if (!_isPlaying) return;

        try
        {
            StatusMessage = "Stopping playback...";
            
            // Generer un ID de requête basique
            string requestId = Guid.NewGuid().ToString();
            
            // Communication avec le backend Rust via le service IPC
            bool success = await _ipcService.SendCommandAsync("Stop", requestId);

            if (success)
            {
                _isPlaying = false;
                PlayButtonText = "Play ▶";
                PlayButtonColor = "#4CAF50";
                EngineStateText = "Stopped";
                EngineStateColor = "#de0013ff";
                StatusMessage = "Engine: Stopped";
            }
            else
            {
                StatusMessage = "Error: Failed to communicate with engine.";
            }
        }
        catch (Exception ex)
        {
            StatusMessage = $"Error: {ex.Message}";
        }
    }

    private async Task SendSetParamAsync (string paramName, double value)
    {
        try
        {
            StatusMessage = $"Setting {paramName} to {value}...";
            
            // Generer un ID de requête basique
            string requestId = Guid.NewGuid().ToString();
            
            // Communication avec le backend Rust via le service IPC
            bool success = await _ipcService.SendCommandAsync("SetParam", requestId, new { name = paramName, value });

            if (success)
            {
                StatusMessage = $"{paramName} set to {value}.";
            }
            else
            {
                StatusMessage = $"Error: Failed to set {paramName}.";
            }
        }
        catch (Exception ex)
        {
            StatusMessage = $"Error: {ex.Message}";
        }
    }
}