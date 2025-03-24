import { z } from "zod";

export const LoginFormSchema = z.object({
  email: z.string().email("Invalid email format"),
  password: z.string().min(8, "Password must be at least 8 characters long"),
  accountType: z.enum(["Patient", "Doctor", "Hospital"], {
    required_error: "Please select an account type",
  }),
});
