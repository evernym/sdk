from distutils.core import setup

setup(
    name='python3-cxs',
    version='0.1',
    packages=['indy'],
    url='https://github.com/evernym/cxs',
    license='MIT/Apache-2.0',
    author='Mark Hadley, Devin Fisher, Ryan Marsh, Doug Wightman',
    author_email='mark.hadley@evernym.com',
    description='Python wrapper for cxs.',
    install_requires=['pytest', 'pytest-asyncio', 'base58'],
    tests_require=['pytest', 'pytest-asyncio', 'base58']
)
