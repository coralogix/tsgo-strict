import type { Type1402 } from '../batch-07/file-1402.js';
export interface Type1403 {
  id: 1403;
  name: 'File1403';
  next: Type1402;
}

export function make1403(): Type1403 {
  return { id: 1403, name: 'File1403', next: null as unknown as Type1402 };
}
