import type { Type1971 } from '../batch-09/file-1971.js';
export interface Type1972 {
  id: 1972;
  name: 'File1972';
  next: Type1971;
}

export function make1972(): Type1972 {
  return { id: 1972, name: 'File1972', next: null as unknown as Type1971 };
}
