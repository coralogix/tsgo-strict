import type { Type1987 } from '../batch-09/file-1987.js';
export interface Type1988 {
  id: 1988;
  name: 'File1988';
  next: Type1987;
}

export function make1988(): Type1988 {
  return { id: 1988, name: 'File1988', next: null as unknown as Type1987 };
}
