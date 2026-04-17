import type { Type1992 } from '../batch-09/file-1992.js';
export interface Type1993 {
  id: 1993;
  name: 'File1993';
  next: Type1992;
}

export function make1993(): Type1993 {
  return { id: 1993, name: 'File1993', next: null as unknown as Type1992 };
}
