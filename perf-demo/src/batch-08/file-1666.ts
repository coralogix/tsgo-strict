import type { Type1665 } from '../batch-08/file-1665.js';
export interface Type1666 {
  id: 1666;
  name: 'File1666';
  next: Type1665;
}

export function make1666(): Type1666 {
  return { id: 1666, name: 'File1666', next: null as unknown as Type1665 };
}
