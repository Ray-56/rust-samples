export interface TodoDTO {
  id: number;
  description: string;
  status: "pending" | "doing" | "completed";
  position: number;
  updated_at: string;
  created_at: string;
}
