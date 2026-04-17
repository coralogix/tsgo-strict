import type { Type444 } from '../batch-02/file-0444.js';
export interface Type445 {
  id: 445;
  name: 'File445';
  next: Type444;
}

export function make445(): Type445 {
  return { id: 445, name: 'File445', next: null as unknown as Type444 };
}
