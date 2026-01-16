export default class Description {
  private constructor(private readonly value: string) {}

  static create(value: string): Description {
    if (!value || value.trim().length === 0) {
      throw new Error("Description cannot be empty");
    }
    if (value.length > 255) {
      throw new Error("Description cannot exceed 255 characters");
    }
    return new Description(value.trim());
  }

  getValue(): string {
    return this.value;
  }

  equals(other: Description): boolean {
    return this.value === other.value;
  }
}