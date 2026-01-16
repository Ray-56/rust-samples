export class Username {
  private constructor(private readonly value: string) {}

  static create(value: string): Username {
    if (!value) {
      throw new Error("Username cannot be empty");
    }
    if (value.trim().length < 3 || value.trim().length > 50) {
      throw new Error("Username must be between 3 and 50 characters");
    }
    return new Username(value.trim());
  }

  getValue(): string {
    return this.value;
  }

  equals(other: Username): boolean {
    if (other instanceof Username) {
      return this.value === other.value;
    }
    return false;
  }
}
