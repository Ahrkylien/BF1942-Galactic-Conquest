subshader "rebtrans_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
	texture "texture/buildings/hoth/EngineGlow_ID_3";
}

subshader "rebtrans_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.650 0.650 0.650;
	envmap true;
	texture "texture/buildings/hoth/hull_ID_1";
}

subshader "rebtrans_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.650 0.650 0.650;
	envmap true;
	texture "texture/buildings/hoth/cargo_ID_2";
}

subshader "rebtrans_Material3" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 0.584314 0.584314 0.584314;
	texture "texture/buildings/hoth/EngineGlow_ID_4";
}

