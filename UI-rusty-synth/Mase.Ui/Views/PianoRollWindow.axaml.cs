using Avalonia.Controls;
using Avalonia.Layout;
using Avalonia.Media;
using Mase.Ui.Controls;
using Mase.Ui.ViewModels;

namespace Mase.Ui.Views;

public class PianoRollWindow : Window
{
    private readonly PianoRollGrid _roll;
    private readonly MainViewModel _vm;

    public PianoRollWindow(MainViewModel vm)
    {
        _vm = vm;

        Title     = "Piano Roll — MASE";
        Width     = 1200;
        Height    = 700;
        MinWidth  = 800;
        MinHeight = 400;
        Background = new SolidColorBrush(Color.Parse("#141414"));

        var grid = new Grid();
        grid.RowDefinitions.Add(new RowDefinition(GridLength.Auto));
        grid.RowDefinitions.Add(new RowDefinition(GridLength.Star));

        var header = new Border
        {
            Background      = new SolidColorBrush(Color.Parse("#2D2D30")),
            BorderBrush     = new SolidColorBrush(Color.Parse("#3E3E42")),
            BorderThickness = new Avalonia.Thickness(0, 0, 0, 1),
            Padding         = new Avalonia.Thickness(10, 6),
            Child           = new TextBlock
            {
                Text       = "PIANO ROLL",
                Foreground = new SolidColorBrush(Color.Parse("#CCCCCC")),
                FontWeight = Avalonia.Media.FontWeight.Bold,
                FontSize   = 12
            }
        };

        _roll = new PianoRollGrid
        {
            BaseNote  = 24, // C1
            NoteCount = 72, // 6 octaves
            Notes     = vm.Notes
        };

        _roll.NoteAdded   += OnNoteAdded;
        _roll.NoteDeleted += OnNoteDeleted;
        _roll.NoteMoved   += OnNoteMoved;
        _roll.NoteResized += OnNoteResized;

        Grid.SetRow(header, 0);
        Grid.SetRow(_roll,  1);

        grid.Children.Add(header);
        grid.Children.Add(_roll);

        Content = grid;
    }

    private async void OnNoteAdded(object? sender, NoteAddedEventArgs e)
    {
        await _vm.AddNoteAsync(e.Pitch, e.Beat);
        _roll.Notes = _vm.Notes;
        _roll.InvalidateVisual();
    }

    private async void OnNoteDeleted(object? sender, NoteDeletedEventArgs e)
    {
        await _vm.DeleteNoteAsync(e.NoteId);
        _roll.Notes = _vm.Notes;
        _roll.InvalidateVisual();
    }

    private async void OnNoteMoved(object? sender, NoteMovedEventArgs e)
    {
        await _vm.MoveNoteAsync(e.NoteId, e.NewPitch, e.NewBeat);
        _roll.InvalidateVisual();
    }

    private async void OnNoteResized(object? sender, NoteResizedEventArgs e)
    {
        await _vm.ResizeNoteAsync(e.NoteId, e.NewLength);
        _roll.InvalidateVisual();
    }
}
