using Mase.Ui.Models;
using Xunit;

namespace Mase.Ui.Tests.Models;

public class NoteModelTests
{
    [Fact]
    public void NewNote_HasNonEmptyId()
    {
        var note = new NoteModel();
        Assert.False(string.IsNullOrEmpty(note.Id));
    }

    [Fact]
    public void TwoNotes_HaveDifferentIds()
    {
        var a = new NoteModel();
        var b = new NoteModel();
        Assert.NotEqual(a.Id, b.Id);
    }

    [Fact]
    public void NewNote_DefaultLengthIsOne()
    {
        Assert.Equal(1.0, new NoteModel().Length);
    }

    [Fact]
    public void NewNote_DefaultVelocityIs0_9()
    {
        Assert.Equal(0.9, new NoteModel().Velocity);
    }

    [Fact]
    public void NewNote_DefaultPitchIsZero()
    {
        Assert.Equal(0, new NoteModel().Pitch);
    }

    [Fact]
    public void NewNote_DefaultStartIsZero()
    {
        Assert.Equal(0, new NoteModel().Start);
    }

    [Fact]
    public void Note_CanSetAllProperties()
    {
        var note = new NoteModel
        {
            Pitch    = 60,
            Start    = 4,
            Length   = 2.0,
            Velocity = 0.5
        };

        Assert.Equal(60,  note.Pitch);
        Assert.Equal(4,   note.Start);
        Assert.Equal(2.0, note.Length);
        Assert.Equal(0.5, note.Velocity);
    }
}
