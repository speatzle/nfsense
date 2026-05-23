export const useMediaQuery = (query: string) => {
  let $matches = false;
  const mediaQueryList = window.matchMedia(query);

  watchEffect(() => {
    $matches = mediaQueryList.matches;
    const listener = (event: MediaQueryListEvent) => ($matches = event.matches);
    mediaQueryList.addEventListener("change", listener);
    return () => mediaQueryList.removeEventListener("change", listener);
  });

  return $$($matches);
};
