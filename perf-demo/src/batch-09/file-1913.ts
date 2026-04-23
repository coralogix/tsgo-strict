import type { Type1912 } from '../batch-09/file-1912.js';
export interface Type1913 {
  id: 1913;
  name: 'File1913';
  next: Type1912;
}

export function make1913(): Type1913 {
  return { id: 1913, name: 'File1913', next: null as unknown as Type1912 };
}
