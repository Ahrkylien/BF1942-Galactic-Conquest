subshader "cammo_Material0" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/cammo";
}

subshader "cammo_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588 0.588 0.588;
	texture "texture/Spruse_bark_T";
}

subshader "cammo_Material2" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	texture "texture/cammo_inside";
}

