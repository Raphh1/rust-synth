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
    private string _playButtonColor = "#4CAF50"; // Light green

    private bool _isPlaying = false;

    [RelayCommand]
    private void OpenPianoRoll()
    {
        StatusMessage = "Opening Piano Roll...";
    }

    [RelayCommand]
    private async Task PlayAsync()
    {
        if (_isPlaying)
        {
            // Already playing, do nothing or handle Stop
            return;
        }

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
                
                // Feedback visuel: changer texte et couleur (gris ou vert plus vif)
                PlayButtonText = "Playing ▶▶";
                PlayButtonColor = "#2E7D32"; // Darker green
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
        if (!_isPlaying)
        {
            // Already stopped, do nothing
            return;
        }

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
                
                // Feedback visuel: revenir a l'etat initial pour le bouton play
                PlayButtonText = "Play ▶";
                PlayButtonColor = "#4CAF50"; // Light green
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
}