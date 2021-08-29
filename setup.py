import platform
from pathlib import Path
from typing import (TYPE_CHECKING,
                    Iterator)

from setuptools import (find_packages,
                        setup)

import rithm

if TYPE_CHECKING:
    from setuptools_rust import RustExtension

project_base_url = 'https://github.com/lycantropos/rithm/'


def read_file(path_string: str) -> str:
    return Path(path_string).read_text(encoding='utf-8')


parameters = dict(
    name=rithm.__name__,
    packages=find_packages(exclude=('tests', 'tests.*')),
    version=rithm.__version__,
    description=rithm.__doc__,
    long_description=read_file('README.md'),
    long_description_content_type='text/markdown',
    author='Azat Ibrakov',
    author_email='azatibrakov@gmail.com',
    classifiers=[
        'License :: OSI Approved :: MIT License',
        'Programming Language :: Python :: 3.6',
        'Programming Language :: Python :: 3.7',
        'Programming Language :: Python :: 3.8',
        'Programming Language :: Python :: 3.9',
        'Programming Language :: Python :: Implementation :: CPython',
        'Programming Language :: Python :: Implementation :: PyPy',
    ],
    license='MIT License',
    url=project_base_url,
    download_url=project_base_url + 'archive/master.zip',
    python_requires='>=3.6',
    setup_requires=read_file('requirements-setup.txt'),
    install_requires=read_file('requirements.txt'))
if platform.python_implementation() == 'CPython':
    class LazyRustExtensions:
        def __iter__(self) -> Iterator['RustExtension']:
            from setuptools_rust import RustExtension
            yield RustExtension(rithm.__name__ + '._' + rithm.__name__)


    parameters.update(rust_extensions=LazyRustExtensions(),
                      include_package_data=True,
                      zip_safe=False)
setup(**parameters)
