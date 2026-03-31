using Avalonia.Controls;
using Avalonia.Layout;
using Avalonia.Media;
using Mase.Ui.Controls;

namespace Mase.Ui.Views;

public class PianoRollWindow : Window
{
    public PianoRollWindow()
    {
        Title     = "Piano Roll — MASE";
        Width     = 1200;
        Height    = 700;
        MinWidth  = 800;
        MinHeight = 400;
        Background = new SolidColorBrush(Avalonia.Media.Color.Parse("#141414"));

        var grid = new Grid();
        grid.RowDefinitions.Add(new RowDefinition(GridLength.Auto));
        grid.RowDefinitions.Add(new RowDefinition(GridLength.Star));

        var header = new Border
        {
            Background      = new SolidColorBrush(Avalonia.Media.Color.Parse("#2D2D30")),
            BorderBrush     = new SolidColorBrush(Avalonia.Media.Color.Parse("#3E3E42")),
            BorderThickness = new Avalonia.Thickness(0, 0, 0, 1),
            Padding         = new Avalonia.Thickness(10, 6),
            Child           = new TextBlock
            {
                Text       = "PIANO ROLL",
                Foreground = new SolidColorBrush(Avalonia.Media.Color.Parse("#CCCCCC")),
                FontWeight = FontWeight.Bold,
                FontSize   = 12
            }
        };

        var roll = new PianoRollGrid
        {
            BaseNote  = 24, // C1
            NoteCount = 72  // 6 octaves
        };

        Grid.SetRow(header, 0);
        Grid.SetRow(roll,   1);

        grid.Children.Add(header);
        grid.Children.Add(roll);

        Content = grid;
    }
}
