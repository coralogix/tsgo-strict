import type { Type1306 } from '../batch-06/file-1306.js';
export interface Type1307 {
  id: 1307;
  name: 'File1307';
  next: Type1306;
}

export function make1307(): Type1307 {
  return { id: 1307, name: 'File1307', next: null as unknown as Type1306 };
}
