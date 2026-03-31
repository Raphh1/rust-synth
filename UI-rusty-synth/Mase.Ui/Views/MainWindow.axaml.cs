using Avalonia.Controls;
using Mase.Ui.Controls;
using Mase.Ui.ViewModels;

namespace Mase.Ui.Views;

public partial class MainWindow : Window
{
    public MainWindow()
    {
        InitializeComponent();
        MainPianoRoll.NoteAdded += OnNoteAdded;
    }

    private async void OnNoteAdded(object? sender, NoteAddedEventArgs e)
    {
        if (DataContext is not MainViewModel vm) return;
        await vm.AddNoteAsync(e.Pitch, e.Beat);
        MainPianoRoll.Notes = vm.Notes;
        MainPianoRoll.InvalidateVisual();
    }
}
