---
paths:
  - "**/*.tsx"
  - "**/*.jsx"
  - "**/*.vue"
  - "**/*.svelte"
  - "**/*.astro"
---

# Tailwind CSS 规范

## 类名组织

使用 `cn()` 按语义分行, **禁止单行堆砌**:

```tsx
<div
  className={cn(
    "flex items-center gap-2", // 布局
    "w-full h-10 px-4", // 尺寸间距
    "rounded-lg border", // 边框圆角
    "bg-background text-foreground", // 颜色
    "hover:bg-accent", // 状态
    isActive && "bg-accent", // 条件样式放最后
  )}
/>
```

## 组件变体

多变体组件使用 CVA:

```tsx
const button = cva("inline-flex items-center justify-center", {
  variants: {
    variant: { primary: "bg-primary", secondary: "bg-secondary" },
    size: { sm: "h-8 px-3", md: "h-10 px-4" },
  },
  defaultVariants: { variant: "primary", size: "md" },
});
```

## 响应式

移动优先: `w-full md:w-1/2 lg:w-1/3`
