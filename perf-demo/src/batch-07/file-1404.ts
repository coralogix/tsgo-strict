import type { Type1403 } from '../batch-07/file-1403.js';
export interface Type1404 {
  id: 1404;
  name: 'File1404';
  next: Type1403;
}

export function make1404(): Type1404 {
  return { id: 1404, name: 'File1404', next: null as unknown as Type1403 };
}
