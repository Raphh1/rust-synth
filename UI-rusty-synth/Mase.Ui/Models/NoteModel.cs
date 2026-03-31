namespace Mase.Ui.Models;

public class NoteModel
{
    public string Id       { get; set; } = System.Guid.NewGuid().ToString();
    public int    Pitch    { get; set; }
    public int    Start    { get; set; }
    public int    Length   { get; set; } = 1;
    public double Velocity { get; set; } = 0.9;
}
