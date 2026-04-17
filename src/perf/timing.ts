export class Timer {
  private readonly starts = new Map<string, number>();
  private readonly durations = new Map<string, number>();

  start(label: string): void {
    this.starts.set(label, Date.now());
  }

  end(label: string): number {
    const started = this.starts.get(label);
    if (started === undefined) {
      return 0;
    }
    const duration = Date.now() - started;
    this.durations.set(label, duration);
    this.starts.delete(label);
    return duration;
  }

  get(label: string): number {
    return this.durations.get(label) ?? 0;
  }

  entries(): Array<{ label: string; durationMs: number }> {
    return Array.from(this.durations.entries()).map(([label, durationMs]) => ({
      label,
      durationMs
    }));
  }
}
