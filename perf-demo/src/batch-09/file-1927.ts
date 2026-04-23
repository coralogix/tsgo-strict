import type { Type1926 } from '../batch-09/file-1926.js';
export interface Type1927 {
  id: 1927;
  name: 'File1927';
  next: Type1926;
}

export function make1927(): Type1927 {
  return { id: 1927, name: 'File1927', next: null as unknown as Type1926 };
}
