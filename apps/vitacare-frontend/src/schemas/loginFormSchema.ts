import { z } from "zod";

export const LoginFormSchema = z.object({
  email: z.string().nonempty("Email is required").email("Invalid email format"),
  password: z
    .string()
    .nonempty("Password is required")
    .min(8, "Password must be at least 8 characters long"),
  accountType: z.enum(["Patient", "Doctor", "Hospital"], {
    required_error: "Please select an account type",
  }),
});
