import type { Type3601 } from '../batch-18/file-3601.js';
export interface Type3602 {
  id: 3602;
  name: 'File3602';
  next: Type3601;
}

export function make3602(): Type3602 {
  return { id: 3602, name: 'File3602', next: null as unknown as Type3601 };
}
