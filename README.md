import { useState, useEffect } from 'react';

export default function App() {
  const [activeTab, setActiveTab] = useState('about');
  const [isDarkMode, setIsDarkMode] = useState(false);

  // Toggle dark mode
  useEffect(() => {
    if (isDarkMode) {
      document.documentElement.classList.add('dark');
    } else {
      document.documentElement.classList.remove('dark');
    }
  }, [isDarkMode]);

  return (
    <div className={`min-h-screen transition-colors duration-300 ${isDarkMode ? 'bg-gray-900 text-white' : 'bg-gray-50 text-gray-900'}`}>
      {/* Header */}
      <header className={`py-6 px-4 sm:px-6 lg:px-8 border-b ${isDarkMode ? 'border-gray-700 bg-gray-800' : 'border-gray-200 bg-white'}`}>
        <div className="max-w-7xl mx-auto flex justify-between items-center">
          <h1 className="text-2xl font-bold">proxy-https-serv</h1>
          <div className="flex items-center space-x-4">
            <button 
              onClick={() => setIsDarkMode(!isDarkMode)}
              className={`p-2 rounded-full ${isDarkMode ? 'bg-gray-700 hover:bg-gray-600' : 'bg-gray-200 hover:bg-gray-300'}`}
            >
              {isDarkMode ? (
                <SunIcon className="w-5 h-5" />
              ) : (
                <MoonIcon className="w-5 h-5" />
              )}
            </button>
            <a href="#" className={`inline-flex items-center px-4 py-2 rounded-md text-sm font-medium ${isDarkMode ? 'bg-green-600 hover:bg-green-700' : 'bg-green-500 hover:bg-green-600'} text-white transition-colors`}>
              Fork
            </a>
          </div>
        </div>
      </header>

      {/* Navigation Tabs */}
      <nav className={`border-b ${isDarkMode ? 'border-gray-700 bg-gray-800' : 'border-gray-200 bg-white'}`}>
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex space-x-8">
            {['about', 'repository', 'releases', 'packages'].map((tab) => (
              <button
                key={tab}
                onClick={() => setActiveTab(tab)}
                className={`py-4 px-1 font-medium text-sm capitalize transition-colors ${
                  activeTab === tab
                    ? isDarkMode
                      ? 'border-b-2 border-green-500 text-green-400'
                      : 'border-b-2 border-green-600 text-green-600'
                    : isDarkMode
                    ? 'text-gray-300 hover:text-white'
                    : 'text-gray-500 hover:text-gray-900'
                }`}
              >
                {tab}
              </button>
            ))}
          </div>
        </div>
      </nav>

      {/* Main Content */}
      <main className="max-w-7xl mx-auto py-10 px-4 sm:px-6 lg:px-8">
        {/* About Tab */}
        {activeTab === 'about' && (
          <section className="animate-fadeIn">
            <h2 className="text-3xl font-extrabold mb-6">About This Project</h2>
            <div className={`prose max-w-none ${isDarkMode ? 'prose-invert' : ''}`}>
              <p className="text-lg mb-6">
                🌟 <strong>proxy-https-serv</strong> — это мега разьебный проект, созданный для решения одной конкретной задачи:
                <br />
                <span className="text-xl mt-2 block">Создание мощного и гибкого HTTPS-прокси сервера.</span>
              </p>
              
              <p className="mb-6">
                Этот сервер был разработан с нуля как эксперимент в поисках идеального баланса между скоростью, надежностью и простотой использования.
                Он может использоваться как база для создания собственного прокси-сервиса или как учебный материал для понимания работы сетевых протоколов.
              </p>

              <h3 className="text-2xl font-bold mt-8 mb-4">Особенности</h3>
              <ul className="space-y-2 list-disc pl-5">
                <li>⚡ Высокая производительность на основе Node.js</li>
                <li>🔒 Полная поддержка HTTPS</li>
                <li>🔧 Простая конфигурация через JSON</li>
                <li>🧩 Модульная архитектура</li>
                <li>📊 Встроенные метрики и логирование</li>
                <li>🤖 Поддержка современных стандартов</li>
              </ul>

              <h3 className="text-2xl font-bold mt-8 mb-4">Почему именно этот прокси?</h3>
              <p>
                Мы не просто создали еще один прокси. Это <em>настоящее произведение искусства</em>, написанное с любовью к коду и вниманием к деталям.
                Каждая строка была тщательно продумана, каждая функция проверена временем и нагрузкой.
              </p>

              <blockquote className={`mt-6 p-4 border-l-4 ${isDarkMode ? 'border-green-500 bg-gray-800' : 'border-green-600 bg-gray-50'}`}>
                <p>"Если ты хочешь создать что-то действительно стоящее — начни с понимания основ."</p>
                <footer className="mt-2 text-sm opacity-75">— LutiDrist, автор проекта</footer>
              </blockquote>

              <div className="mt-10 grid grid-cols-1 md:grid-cols-3 gap-6">
                <div className={`p-6 rounded-lg shadow ${isDarkMode ? 'bg-gray-800' : 'bg-white'}`}>
                  <h4 className="font-bold text-xl mb-2">Начало работы</h4>
                  <p className="mb-4">Установите зависимости и запустите сервер:</p>
                  <pre className={`p-3 rounded text-sm overflow-x-auto ${isDarkMode ? 'bg-gray-900' : 'bg-gray-100'}`}>
                    <code>npm install && npm start</code>
                  </pre>
                </div>
                <div className={`p-6 rounded-lg shadow ${isDarkMode ? 'bg-gray-800' : 'bg-white'}`}>
                  <h4 className="font-bold text-xl mb-2">Конфигурация</h4>
                  <p className="mb-4">Настройте сервер в файле config.json:</p>
                  <pre className={`p-3 rounded text-sm overflow-x-auto ${isDarkMode ? 'bg-gray-900' : 'bg-gray-100'}`}>
                    <code>{`{
  "port": 8080,
  "target": "https://example.com" 
}`}</code>
                  </pre>
                </div>
                <div className={`p-6 rounded-lg shadow ${isDarkMode ? 'bg-gray-800' : 'bg-white'}`}>
                  <h4 className="font-bold text-xl mb-2">Разворачивание</h4>
                  <p className="mb-4">Используйте Docker для деплоя:</p>
                  <pre className={`p-3 rounded text-sm overflow-x-auto ${isDarkMode ? 'bg-gray-900' : 'bg-gray-100'}`}>
                    <code>docker build -t proxy-server .<br />docker run -p 8080:8080 proxy-server</code>
                  </pre>
                </div>
              </div>
            </div>
          </section>
        )}

        {/* Repository Tab */}
        {activeTab === 'repository' && (
          <section className="animate-fadeIn">
            <h2 className="text-3xl font-extrabold mb-6">Repository Structure</h2>
            
            <div className={`overflow-hidden rounded-lg shadow ${isDarkMode ? 'bg-gray-800' : 'bg-white'}`}>
              <table className="min-w-full divide-y divide-gray-200">
                <thead className={`${isDarkMode ? 'bg-gray-700' : 'bg-gray-50'}`}>
                  <tr>
                    <th scope="col" className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Name</th>
                    <th scope="col" className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Last Commit Message</th>
                    <th scope="col" className="px-6 py-3 text-left text-xs font-medium uppercase tracking-wider">Date</th>
                  </tr>
                </thead>
                <tbody className={`divide-y ${isDarkMode ? 'divide-gray-700' : 'divide-gray-200'}`}>
                  <tr>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">README.md</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">Update README with mega cool description</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">2023-08-15</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">server.js</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">Initial commit</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">2023-08-10</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">package.json</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">Add dependencies</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">2023-08-12</td>
                  </tr>
                  <tr>
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">config.example.json</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">Add example config</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">2023-08-14</td>
                  </tr>
                </tbody>
              </table>
            </div>

            <div className="mt-10">
              <h3 className="text-2xl font-bold mb-4">Resources</h3>
              <div className={`p-6 rounded-lg ${isDarkMode ? 'bg-gray-800' : 'bg-white'} shadow`}>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                  <div className="flex flex-col items-center text-center">
                    <StarIcon className={`w-10 h-10 mb-2 ${isDarkMode ? 'text-yellow-400' : 'text-yellow-500'}`} />
                    <h4 className="font-bold text-lg mb-2">Stars</h4>
                    <p className="text-3xl font-bold">0</p>
                  </div>
                  <div className="flex flex-col items-center text-center">
                    <EyeIcon className={`w-10 h-10 mb-2 ${isDarkMode ? 'text-blue-400' : 'text-blue-500'}`} />
                    <h4 className="font-bold text-lg mb-2">Watchers</h4>
                    <p className="text-3xl font-bold">0</p>
                  </div>
                  <div className="flex flex-col items-center text-center">
                    <GitForkIcon className={`w-10 h-10 mb-2 ${isDarkMode ? 'text-purple-400' : 'text-purple-500'}`} />
                    <h4 className="font-bold text-lg mb-2">Forks</h4>
                    <p className="text-3xl font-bold">0</p>
                  </div>
                </div>
              </div>
            </div>
          </section>
        )}

        {/* Releases Tab */}
        {activeTab === 'releases' && (
          <section className="animate-fadeIn">
            <h2 className="text-3xl font-extrabold mb-6">Releases</h2>
            <div className={`p-8 rounded-lg text-center ${isDarkMode ? 'bg-gray-800' : 'bg-white'} shadow`}>
              <BoxIcon className={`w-16 h-16 mx-auto mb-4 ${isDarkMode ? 'text-gray-600' : 'text-gray-300'}`} />
              <h3 className="text-xl font-semibold mb-2">No releases published</h3>
              <p className="mb-6">When you create a new release, it will appear here.</p>
              <button className={`inline-flex items-center px-4 py-2 rounded-md text-sm font-medium ${isDarkMode ? 'bg-green-600 hover:bg-green-700' : 'bg-green-500 hover:bg-green-600'} text-white transition-colors`}>
                Create your first release
              </button>
            </div>
          </section>
        )}

        {/* Packages Tab */}
        {activeTab === 'packages' && (
          <section className="animate-fadeIn">
            <h2 className="text-3xl font-extrabold mb-6">Packages</h2>
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
              <div className={`p-6 rounded-lg shadow ${isDarkMode ? 'bg-gray-800' : 'bg-white'}`}>
                <h3 className="text-xl font-semibold mb-4">NPM Packages</h3>
                <p className="mb-4">This repository doesn't publish any NPM packages.</p>
                <button className={`inline-flex items-center px-4 py-2 rounded-md text-sm font-medium ${isDarkMode ? 'bg-gray-700 hover:bg-gray-600' : 'bg-gray-200 hover:bg-gray-300'} transition-colors`}>
                  Publish package
                </button>
              </div>
              <div className={`p-6 rounded-lg shadow ${isDarkMode ? 'bg-gray-800' : 'bg-white'}`}>
                <h3 className="text-xl font-semibold mb-4">GitHub Packages</h3>
                <p className="mb-4">This repository doesn't publish any GitHub packages.</p>
                <button className={`inline-flex items-center px-4 py-2 rounded-md text-sm font-medium ${isDarkMode ? 'bg-gray-700 hover:bg-gray-600' : 'bg-gray-200 hover:bg-gray-300'} transition-colors`}>
                  Configure packages
                </button>
              </div>
            </div>
          </section>
        )}
      </main>

      {/* Footer */}
      <footer className={`py-8 border-t ${isDarkMode ? 'border-gray-800 bg-gray-900' : 'border-gray-200 bg-gray-100'}`}>
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
          <div className="flex flex-col items-center justify-center">
            <p className={`text-sm ${isDarkMode ? 'text-gray-400' : 'text-gray-600'}`}>
              &copy; {new Date().getFullYear()} proxy-https-serv. All rights reserved.
            </p>
            <div className="flex space-x-6 mt-4">
              <a href="#" className={`hover:${isDarkMode ? 'text-green-400' : 'text-green-600'} transition-colors`}>Privacy Policy</a>
              <a href="#" className={`hover:${isDarkMode ? 'text-green-400' : 'text-green-600'} transition-colors`}>Terms of Service</a>
              <a href="#" className={`hover:${isDarkMode ? 'text-green-400' : 'text-green-600'} transition-colors`}>Contact</a>
            </div>
          </div>
        </div>
      </footer>

      <style jsx>{`
        @keyframes fadeIn {
          from { opacity: 0; transform: translateY(10px); }
          to { opacity: 1; transform: translateY(0); }
        }
        .animate-fadeIn {
          animation: fadeIn 0.5s ease-out forwards;
        }
      `}</style>
    </div>
  );
}

// Icons
function SunIcon({ className }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
    </svg>
  );
}

function MoonIcon({ className }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
    </svg>
  );
}

function StarIcon({ className }) {
  return (
    <svg className={className} fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg">
      <path d="M9.049 2.927c.3-.921 1.603-.921 1.902 0l1.07 3.292a1 1 0 00.95.69h3.462c.969 0 1.371 1.24.588 1.81l-2.8 2.034a1 1 0 00-.364 1.118l1.07 3.292c.3.921-.755 1.688-1.54 1.118l-2.8-2.034a1 1 0 00-1.175 0l-2.8 2.034c-.784.57-1.838-.197-1.539-1.118l1.07-3.292a1 1 0 00-.364-1.118L2.98 8.72c-.783-.57-.38-1.81.588-1.81h3.461a1 1 0 00.951-.69l1.07-3.292z" />
    </svg>
  );
}

function EyeIcon({ className }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
    </svg>
  );
}

function GitForkIcon({ className }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" />
    </svg>
  );
}

function BoxIcon({ className }) {
  return (
    <svg className={className} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
      <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M20 7l-8-4-8 4m16 0l-8 4m8-4v10l-8 4m0-10L4 7m8 4v10M4 7v10l8 4" />
    </svg>
  );
}
