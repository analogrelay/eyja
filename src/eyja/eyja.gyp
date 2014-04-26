{
	'targets': [
		{
			## eyja console app
			'target_name': 'eyja',
			'type': 'executable',
			'msvs_guid': 'D913FA4F-3B77-4D90-B6E8-7BAE70C8CEA8',
			'dependencies': [
			],
			'conditions': [],
			'include_dirs': [
			],
			'sources': [
                './main.cpp',
				'./precompiled.h',
				'./precompiled.cpp',
				'./store.h',
                './store.cpp',
                './value.h',
                './value.cpp',
				'./XGetopt.h',
				'./XGetopt.cpp'
			]
		}
	]
}