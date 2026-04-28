export function usePlugins() {
  const router = useRouter();
  const toast = useToast();

  return { router, toast };
}
