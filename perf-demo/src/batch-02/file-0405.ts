import type { Type404 } from '../batch-02/file-0404.js';
export interface Type405 {
  id: 405;
  name: 'File405';
  next: Type404;
}

export function make405(): Type405 {
  return { id: 405, name: 'File405', next: null as unknown as Type404 };
}
