import { useForm } from 'react-hook-form';
import { zodResolver } from '@hookform/resolvers/zod';
import * as z from 'zod';

import { Button } from '@/presentation/components/ui/button';
import { Input } from '@/presentation/components/ui/input';
import { Alert } from '@/presentation/components/ui/alert';

const schema = z.object({
  username: z.string().min(3, 'Username must be at least 3 characters').max(50, 'Username must be at most 50 characters'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
  confirmPassword: z.string().min(6, 'Confirm Password must be at least 6 characters'),
}).refine((data) => data.password === data.confirmPassword, {
  message: 'Passwords do not match',
  path: ['confirmPassword'],
});

interface RegisterFormProps {
  onSubmit: (data: { username: string; password: string }) => Promise<void>;
  error?: string | null;
}

export function RegisterForm({ onSubmit, error }: RegisterFormProps) {
  const { register, handleSubmit, formState: { errors, isSubmitting } } = useForm({
    resolver: zodResolver(schema),
  });

  const handleFormSubmit = (data: any) => {
    onSubmit({ username: data.username, password: data.password });
  };

  return (
    <form onSubmit={handleSubmit(handleFormSubmit)} className="space-y-4">
      {error && <Alert variant="destructive">{error}</Alert>}
      <div>
        <Input
          type="text"
          placeholder="Username"
          {...register('username')}
          className={errors.username ? 'border-red-500' : ''}
        />
        {errors.username && <p className="text-red-500 text-sm">{errors.username.message}</p>}
      </div>
      <div>
        <Input
          type="password"
          placeholder="Password"
          {...register('password')}
          className={errors.password ? 'border-red-500' : ''}
        />
        {errors.password && <p className="text-red-500 text-sm">{errors.password.message}</p>}
      </div>
      <div>
        <Input
          type="password"
          placeholder="Confirm Password"
          {...register('confirmPassword')}
          className={errors.confirmPassword ? 'border-red-500' : ''}
        />
        {errors.confirmPassword && <p className="text-red-500 text-sm">{errors.confirmPassword.message}</p>}
      </div>
      <Button type="submit" disabled={isSubmitting} className="w-full">
        {isSubmitting ? 'Logging in...' : 'Login'}
      </Button>
    </form>
  );
}