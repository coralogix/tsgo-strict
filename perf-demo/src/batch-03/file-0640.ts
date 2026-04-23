import type { Type639 } from '../batch-03/file-0639.js';
export interface Type640 {
  id: 640;
  name: 'File640';
  next: Type639;
}

export function make640(): Type640 {
  return { id: 640, name: 'File640', next: null as unknown as Type639 };
}
