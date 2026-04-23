import type { Type1420 } from '../batch-07/file-1420.js';
export interface Type1421 {
  id: 1421;
  name: 'File1421';
  next: Type1420;
}

export function make1421(): Type1421 {
  return { id: 1421, name: 'File1421', next: null as unknown as Type1420 };
}
