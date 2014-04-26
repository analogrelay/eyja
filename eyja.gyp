{
	'targets': [
		{
			## eyja console app
			'target_name': 'eyja',
			'type': 'executable',
			'msvs_guid': 'D913FA4F-3B77-4D90-B6E8-7BAE70C8CEA8',
			'dependencies': [
				'./ext/Haywire/haywire.gyp:haywire'
			],
			'conditions': [],
			'include_dirs': [
				'./ext/Haywire/include',
				'./ext'
			],
			'sources': [
				
			]
		}
	]
}