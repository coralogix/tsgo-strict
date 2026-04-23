import type { Type403 } from '../batch-02/file-0403.js';
export interface Type404 {
  id: 404;
  name: 'File404';
  next: Type403;
}

export function make404(): Type404 {
  return { id: 404, name: 'File404', next: null as unknown as Type403 };
}
