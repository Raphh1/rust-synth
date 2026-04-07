using Mase.Ui.Models;
using Xunit;

namespace Mase.Ui.Tests.Models;

public class WaveformsTests
{
    private const double Tolerance = 1e-9;

    // ─── Sine ────────────────────────────────────────────────────────────────

    [Fact]
    public void Sine_DefaultLength_Is128()
    {
        var table = Waveforms.Sine();
        Assert.Equal(128, table.Length);
    }

    [Fact]
    public void Sine_StartsAtZero()
    {
        var table = Waveforms.Sine();
        Assert.Equal(0.0, table[0], Tolerance);
    }

    [Fact]
    public void Sine_PeakAtQuarter()
    {
        var table = Waveforms.Sine();
        Assert.Equal(1.0, table[32], Tolerance);
    }

    [Fact]
    public void Sine_BackToZeroAtHalf()
    {
        var table = Waveforms.Sine();
        Assert.Equal(0.0, table[64], Tolerance);
    }

    [Fact]
    public void Sine_TroughAtThreeQuarters()
    {
        var table = Waveforms.Sine();
        Assert.Equal(-1.0, table[96], Tolerance);
    }

    [Fact]
    public void Sine_AllValuesInRange()
    {
        var table = Waveforms.Sine();
        Assert.All(table, v => Assert.InRange(v, -1.0, 1.0));
    }

    [Fact]
    public void Sine_CustomLength()
    {
        var table = Waveforms.Sine(64);
        Assert.Equal(64, table.Length);
        Assert.Equal(1.0, table[16], Tolerance);
    }

    // ─── Square ──────────────────────────────────────────────────────────────

    [Fact]
    public void Square_DefaultLength_Is128()
    {
        Assert.Equal(128, Waveforms.Square().Length);
    }

    [Fact]
    public void Square_FirstHalfIsOne()
    {
        var table = Waveforms.Square();
        for (int i = 0; i < 64; i++)
            Assert.Equal(1.0, table[i]);
    }

    [Fact]
    public void Square_SecondHalfIsMinusOne()
    {
        var table = Waveforms.Square();
        for (int i = 64; i < 128; i++)
            Assert.Equal(-1.0, table[i]);
    }

    [Fact]
    public void Square_OnlyContainsOneAndMinusOne()
    {
        var table = Waveforms.Square();
        Assert.All(table, v => Assert.True(v == 1.0 || v == -1.0));
    }

    // ─── Saw ─────────────────────────────────────────────────────────────────

    [Fact]
    public void Saw_DefaultLength_Is128()
    {
        Assert.Equal(128, Waveforms.Saw().Length);
    }

    [Fact]
    public void Saw_StartsAtOne()
    {
        Assert.Equal(1.0, Waveforms.Saw()[0], Tolerance);
    }

    [Fact]
    public void Saw_EndsAtMinusOne()
    {
        var table = Waveforms.Saw();
        Assert.Equal(-1.0, table[^1], Tolerance);
    }

    [Fact]
    public void Saw_IsMonotonicallyDecreasing()
    {
        var table = Waveforms.Saw();
        for (int i = 1; i < table.Length; i++)
            Assert.True(table[i] < table[i - 1], $"Not decreasing at index {i}");
    }

    [Fact]
    public void Saw_AllValuesInRange()
    {
        Assert.All(Waveforms.Saw(), v => Assert.InRange(v, -1.0, 1.0));
    }

    // ─── Triangle ────────────────────────────────────────────────────────────

    [Fact]
    public void Triangle_DefaultLength_Is128()
    {
        Assert.Equal(128, Waveforms.Triangle().Length);
    }

    [Fact]
    public void Triangle_StartsAtZero()
    {
        Assert.Equal(0.0, Waveforms.Triangle()[0], Tolerance);
    }

    [Fact]
    public void Triangle_PeakAtQuarter()
    {
        var table = Waveforms.Triangle();
        Assert.Equal(1.0, table[32], Tolerance);
    }

    [Fact]
    public void Triangle_BackToZeroAtHalf()
    {
        var table = Waveforms.Triangle();
        Assert.Equal(0.0, table[64], Tolerance);
    }

    [Fact]
    public void Triangle_TroughAtThreeQuarters()
    {
        var table = Waveforms.Triangle();
        Assert.Equal(-1.0, table[96], Tolerance);
    }

    [Fact]
    public void Triangle_AllValuesInRange()
    {
        Assert.All(Waveforms.Triangle(), v => Assert.InRange(v, -1.0, 1.0));
    }
}
