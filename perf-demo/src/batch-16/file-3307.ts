import type { Type3306 } from '../batch-16/file-3306.js';
export interface Type3307 {
  id: 3307;
  name: 'File3307';
  next: Type3306;
}

export function make3307(): Type3307 {
  return { id: 3307, name: 'File3307', next: null as unknown as Type3306 };
}
