import { Button } from "@/presentation/components/ui/button";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/presentation/components/ui/table";
import { Alert } from "@/presentation/components/ui/alert";
import {
  Pagination,
  PaginationContent,
  PaginationItem,
  PaginationLink,
  PaginationNext,
  PaginationPrevious,
} from "@/presentation/components/ui/pagination";
import { Account } from "@/domains/accounts/entities/Account";

interface UserListProps {
  users: Account[];
  isLoading: boolean;
  error: string | null;
  onDelete: (userId: string) => Promise<void>;
  currentUserId?: string;
  total: number;
  page: number;
  pageSize: number;
  setPage: (page: number) => void;
}

export function UserList({
  users,
  isLoading,
  error,
  onDelete,
  currentUserId,
  total,
  page,
  pageSize,
  setPage,
}: UserListProps) {
  const totalPages = Math.ceil(total / pageSize);

  if (isLoading) {
    return <p className="text-center">Loading...</p>;
  }

  if (error) {
    return <Alert variant="destructive">{error}</Alert>;
  }

  if (users.length === 0) {
    return <p className="text-center text-gray-500">No users yet</p>;
  }

  return (
    <div className="space-y-4">
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>Username</TableHead>
            <TableHead>ID</TableHead>
            <TableHead>Actions</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {users.map((user) => (
            <TableRow key={user.getId()}>
              <TableCell>{user.getUsername()}</TableCell>
              <TableCell>{user.getId()}</TableCell>
              <TableCell>
                <Button
                  variant="destructive"
                  size="sm"
                  onClick={() => onDelete(user.getId())}
                  disabled={isLoading || currentUserId === user.getId()}
                >
                  Delete
                </Button>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
      <Pagination>
        <PaginationContent>
          <PaginationItem>
            <PaginationPrevious
              onClick={() => setPage(page - 1)}
              className={
                page <= 1 ? "pointer-events-none opacity-50" : "cursor-pointer"
              }
            />
          </PaginationItem>
          {Array.from({ length: totalPages }, (_, i) => i + 1).map((p) => (
            <PaginationItem key={p}>
              <PaginationLink
                onClick={() => setPage(p)}
                isActive={p === page}
                className={
                  p === page ? "bg-blue-500 text-white" : "cursor-pointer"
                }
              >
                {p}
              </PaginationLink>
            </PaginationItem>
          ))}
          <PaginationItem>
            <PaginationNext
              onClick={() => setPage(page + 1)}
              className={
                page >= totalPages
                  ? "pointer-events-none opacity-50"
                  : "cursor-pointer"
              }
            />
          </PaginationItem>
        </PaginationContent>
      </Pagination>
    </div>
  );
}
