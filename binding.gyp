{
    'targets': [
        {
            'target_name': 'xosms-native',
            'sources': ['src/xosms.cpp'],
            'include_dirs': ["<!@(node -p \"require('node-addon-api').include\")"],
            'dependencies': ["<!(node -p \"require('node-addon-api').gyp\")"],
            'cflags!': ['-fno-exceptions'],
            'cflags_cc!': ['-fno-exceptions'],
            'xcode_settings': {
                'GCC_ENABLE_CPP_EXCEPTIONS': 'YES',
                'CLANG_CXX_LIBRARY': 'libc++',
                'MACOSX_DEPLOYMENT_TARGET': '10.7'
            },
            'configurations': {
                'Debug': {
                    'msvs_settings': {
                        'VCCLCompilerTool': {
                            'ExceptionHandling': 1,  # /EHsc,
                            'RuntimeLibrary': 3,  # /MDd
                        }
                    }
                },
                'Release': {
                    'msvs_settings': {
                        'VCCLCompilerTool': {
                            'ExceptionHandling': 1,  # /EHsc,
                            'RuntimeLibrary': 2,  # /MD
                        }
                    }
                }
            },
            "conditions": [
                ['OS==\'win\'', {
                    'libraries': ['-lruntimeobject.lib'],
                    'sources': [
                        'src/win/media_service.cpp',
                        'src/win/utils.cpp'
                    ],
                    "msvs_settings": {
                        "VCCLCompilerTool": {
                            "AdditionalOptions": ["/ZW"],
                            "AdditionalUsingDirectories": [
                                "%ProgramFiles(x86)%/Microsoft Visual Studio 14.0/VC/lib/store/references",
                                "%ProgramFiles%/Microsoft Visual Studio 14.0/VC/lib/store/references",
                                "%ProgramFiles%/Windows Kits/10/UnionMetadata/10.0.17134.0",
                                "%ProgramFiles%/Windows Kits/10/Include/10.0.17134.0/um",
                                "%ProgramFiles(x86)%/Windows Kits/10/UnionMetadata/10.0.17134.0",
                                "%ProgramFiles(x86)%/Windows Kits/10/Include/10.0.17134.0/um"
                            ]
                        }
                    }
                }]
            ]
        }
    ]
}
