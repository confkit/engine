// CCDD 工作流系统 - 主要交互逻辑

document.addEventListener('DOMContentLoaded', function() {
    // 平滑滚动导航
    const navLinks = document.querySelectorAll('nav a[href^="#"]');
    navLinks.forEach(link => {
        link.addEventListener('click', function(e) {
            e.preventDefault();
            const targetId = this.getAttribute('href');
            const targetSection = document.querySelector(targetId);
            if (targetSection) {
                targetSection.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });

    // 命令卡片悬停效果
    const commandCards = document.querySelectorAll('[class*="bg-gradient-to-br"]');
    commandCards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            this.style.transform = 'translateY(-4px)';
            this.style.transition = 'transform 0.3s ease';
        });
        
        card.addEventListener('mouseleave', function() {
            this.style.transform = 'translateY(0)';
        });
    });

    // 工作流步骤动画
    const workflowSteps = document.querySelectorAll('[class*="bg-blue-50"], [class*="bg-green-50"]');
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -100px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach(entry => {
            if (entry.isIntersecting) {
                entry.target.style.opacity = '1';
                entry.target.style.transform = 'translateX(0)';
                entry.target.style.transition = 'opacity 0.6s ease, transform 0.6s ease';
            }
        });
    }, observerOptions);

    workflowSteps.forEach(step => {
        step.style.opacity = '0.7';
        step.style.transform = 'translateX(20px)';
        observer.observe(step);
    });

    // 代码块复制功能
    const codeElements = document.querySelectorAll('code');
    codeElements.forEach(code => {
        if (code.textContent.startsWith('/dd:')) {
            code.style.cursor = 'pointer';
            code.title = '点击复制命令';
            
            code.addEventListener('click', function() {
                navigator.clipboard.writeText(this.textContent).then(() => {
                    // 显示复制成功提示
                    showToast('命令已复制到剪贴板');
                });
            });
        }
    });

    // 响应式导航菜单
    const mobileMenuButton = createMobileMenuButton();
    const nav = document.querySelector('nav');
    const header = document.querySelector('header .container > div');
    
    if (nav && window.innerWidth < 768) {
        header.appendChild(mobileMenuButton);
    }

    // 滚动时头部样式变化
    let lastScrollY = window.scrollY;
    window.addEventListener('scroll', () => {
        const header = document.querySelector('header');
        if (window.scrollY > 100) {
            header.classList.add('backdrop-blur-sm', 'bg-white/90');
        } else {
            header.classList.remove('backdrop-blur-sm', 'bg-white/90');
        }
        lastScrollY = window.scrollY;
    });
});

// 创建移动端菜单按钮
function createMobileMenuButton() {
    const button = document.createElement('button');
    button.className = 'md:hidden p-2 rounded-lg hover:bg-gray-100 transition-colors';
    button.innerHTML = `
        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
        </svg>
    `;
    
    button.addEventListener('click', function() {
        const nav = document.querySelector('nav');
        nav.classList.toggle('hidden');
        nav.classList.toggle('absolute');
        nav.classList.toggle('top-full');
        nav.classList.toggle('left-0');
        nav.classList.toggle('w-full');
        nav.classList.toggle('bg-white');
        nav.classList.toggle('shadow-lg');
        nav.classList.toggle('p-4');
    });
    
    return button;
}

// 显示 Toast 提示
function showToast(message) {
    // 移除现有的 toast
    const existingToast = document.querySelector('.toast');
    if (existingToast) {
        existingToast.remove();
    }

    const toast = document.createElement('div');
    toast.className = 'toast fixed top-20 right-4 bg-gray-800 text-white px-4 py-2 rounded-lg shadow-lg z-50 transform translate-x-full transition-transform duration-300';
    toast.textContent = message;
    
    document.body.appendChild(toast);
    
    // 显示动画
    setTimeout(() => {
        toast.classList.remove('translate-x-full');
    }, 10);
    
    // 自动隐藏
    setTimeout(() => {
        toast.classList.add('translate-x-full');
        setTimeout(() => {
            toast.remove();
        }, 300);
    }, 2000);
}

// 特性卡片动画
function animateFeatureCards() {
    const featureCards = document.querySelectorAll('#features .text-center');
    featureCards.forEach((card, index) => {
        setTimeout(() => {
            card.style.opacity = '1';
            card.style.transform = 'translateY(0)';
        }, index * 100);
    });
}

// 页面加载完成后的初始化
window.addEventListener('load', function() {
    // 添加页面加载动画
    document.body.style.opacity = '0';
    document.body.style.transition = 'opacity 0.5s ease';
    
    setTimeout(() => {
        document.body.style.opacity = '1';
    }, 100);
    
    // 延迟执行特性卡片动画
    setTimeout(animateFeatureCards, 1000);
});

// 工具函数：检测是否为移动设备
function isMobile() {
    return window.innerWidth < 768;
}

// 工具函数：节流
function throttle(func, limit) {
    let inThrottle;
    return function() {
        const args = arguments;
        const context = this;
        if (!inThrottle) {
            func.apply(context, args);
            inThrottle = true;
            setTimeout(() => inThrottle = false, limit);
        }
    };
}