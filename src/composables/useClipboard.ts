export function useClipboard() {
    const copyTextToClipboard = async (text: string) => {
      try {
        await navigator.clipboard.writeText(text);
      } catch (err) {
        console.error('Failed to copy text:', err);
      }
    };
  
    const readTextFromClipboard = async (): Promise<string | null> => {
      try {
        const text = await navigator.clipboard.readText();
        return text;
      } catch (err) {
        console.error('Failed to read clipboard contents:', err);
        return null;
      }
    };
  
    return {
      copyTextToClipboard,
      readTextFromClipboard,
    };
  }
  