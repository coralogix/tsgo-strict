import type { Type1954 } from '../batch-09/file-1954.js';
export interface Type1955 {
  id: 1955;
  name: 'File1955';
  next: Type1954;
}

export function make1955(): Type1955 {
  return { id: 1955, name: 'File1955', next: null as unknown as Type1954 };
}
