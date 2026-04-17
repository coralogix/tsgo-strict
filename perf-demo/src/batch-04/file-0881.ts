import type { Type880 } from '../batch-04/file-0880.js';
export interface Type881 {
  id: 881;
  name: 'File881';
  next: Type880;
}

export function make881(): Type881 {
  return { id: 881, name: 'File881', next: null as unknown as Type880 };
}
