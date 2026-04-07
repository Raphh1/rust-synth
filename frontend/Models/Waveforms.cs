using System;
using System.Linq;

namespace Mase.Ui.Models;

public static class Waveforms
{
    public static double[] Sine(int n = 128) =>
        Enumerable.Range(0, n)
            .Select(i => Math.Sin(2 * Math.PI * i / n))
            .ToArray();

    public static double[] Square(int n = 128) =>
        Enumerable.Range(0, n)
            .Select(i => i < n / 2 ? 1.0 : -1.0)
            .ToArray();

    public static double[] Saw(int n = 128) =>
        Enumerable.Range(0, n)
            .Select(i => 1.0 - 2.0 * i / (n - 1))
            .ToArray();

    public static double[] Triangle(int n = 128) =>
        Enumerable.Range(0, n)
            .Select(i =>
            {
                double t = (double)i / n;
                return t < 0.25 ? 4 * t
                     : t < 0.75 ? 2 - 4 * t
                     :            4 * t - 4;
            })
            .ToArray();
}
