import type { Type1972 } from '../batch-09/file-1972.js';
export interface Type1973 {
  id: 1973;
  name: 'File1973';
  next: Type1972;
}

export function make1973(): Type1973 {
  return { id: 1973, name: 'File1973', next: null as unknown as Type1972 };
}
