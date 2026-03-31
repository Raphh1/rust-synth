using Avalonia;
using Avalonia.Controls.ApplicationLifetimes;
using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;
using Mase.Ui.Models;
using Mase.Ui.Services;
using Mase.Ui.Views;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;

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

    public List<NoteModel> Notes { get; } = new();

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
    CutoffDisplay = $"Cutoff: {value:F0} Hz";
}

    [RelayCommand]
    private void OpenPianoRoll()
    {
        var window = new PianoRollWindow(this);
        if (Application.Current?.ApplicationLifetime is IClassicDesktopStyleApplicationLifetime { MainWindow: { } main })
            window.Show(main);
        else
            window.Show();

        StatusMessage = "Piano Roll ouvert.";
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

    public async Task AddNoteAsync(int pitch, int beat)
    {
        try
        {
            var note = new NoteModel { Pitch = pitch, Start = beat };
            string requestId = Guid.NewGuid().ToString();

            bool success = await _ipcService.SendCommandAsync("AddNote", requestId, new { note = new { pitch, start = beat, length = note.Length, velocity = note.Velocity } });

            if (success)
            {
                Notes.Add(note);
                StatusMessage = $"Note ajoutée : pitch {pitch}, beat {beat}";
            }
            else
            {
                StatusMessage = "Error: Failed to add note.";
            }
        }
        catch (Exception ex)
        {
            StatusMessage = $"Error: {ex.Message}";
        }
    }

    public async Task DeleteNoteAsync(string noteId)
    {
        try
        {
            var note = Notes.Find(n => n.Id == noteId);
            if (note == null) return;

            bool success = await _ipcService.SendCommandAsync("DeleteNote", Guid.NewGuid().ToString(),
                new { id = noteId });

            if (success)
            {
                Notes.Remove(note);
                StatusMessage = "Note supprimée.";
            }
        }
        catch (Exception ex) { StatusMessage = $"Error: {ex.Message}"; }
    }

    public async Task MoveNoteAsync(string noteId, int newPitch, int newBeat)
    {
        try
        {
            bool success = await _ipcService.SendCommandAsync("MoveNote", Guid.NewGuid().ToString(),
                new { id = noteId, pitch = newPitch, start = newBeat });

            if (success)
                StatusMessage = $"Note déplacée : pitch {newPitch}, beat {newBeat}";
        }
        catch (Exception ex) { StatusMessage = $"Error: {ex.Message}"; }
    }

    public async Task ResizeNoteAsync(string noteId, double newLength)
    {
        try
        {
            bool success = await _ipcService.SendCommandAsync("ResizeNote", Guid.NewGuid().ToString(),
                new { id = noteId, length = newLength });

            if (success)
                StatusMessage = $"Note redimensionnée : {newLength} beat(s)";
        }
        catch (Exception ex) { StatusMessage = $"Error: {ex.Message}"; }
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