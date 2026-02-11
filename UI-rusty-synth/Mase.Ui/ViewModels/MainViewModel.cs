using CommunityToolkit.Mvvm.ComponentModel;
using CommunityToolkit.Mvvm.Input;

namespace Mase.Ui.ViewModels;

public partial class MainViewModel : ViewModelBase
{
    [ObservableProperty]
    private string _windowTitle = "Rusty Synth - Main Rack";

    [ObservableProperty]
    private string _statusMessage = "Engine: Ready";

    [RelayCommand]
    private void OpenPianoRoll()
    {
        StatusMessage = "Opening Piano Roll...";
    }
}