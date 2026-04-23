import type { Type1986 } from '../batch-09/file-1986.js';
export interface Type1987 {
  id: 1987;
  name: 'File1987';
  next: Type1986;
}

export function make1987(): Type1987 {
  return { id: 1987, name: 'File1987', next: null as unknown as Type1986 };
}
