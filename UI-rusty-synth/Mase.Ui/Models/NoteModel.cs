namespace Mase.Ui.Models;

public class NoteModel
{
    public int    Pitch    { get; set; }
    public int    Start    { get; set; } // en beats
    public int    Length   { get; set; } = 1;
    public double Velocity { get; set; } = 0.9;
}
