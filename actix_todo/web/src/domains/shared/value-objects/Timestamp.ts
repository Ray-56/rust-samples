export default class Timestamp {
  private constructor(private readonly value: Date) {}

  static create(value: string| Date): Timestamp {
    const date = typeof value === "string" ? new Date(value) : value;
    if (isNaN(date.getTime())) {
      throw new Error("Invalid date");
    }
    return new Timestamp(date);
  }

  static now(): Timestamp {
    return new Timestamp(new Date());
  }

  getValue(): Date {
    return this.value;
  }

  toISOString(): string {
    return this.value.toISOString();
  }
}