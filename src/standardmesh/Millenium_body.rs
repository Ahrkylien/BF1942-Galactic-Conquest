subshader "millenium_body_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular true;
	materialDiffuse 1 1 1;
	materialSpecular 0.12549 0.12549 0.12549;
	materialSpecularPower 12.5;
	transparent true;
	twosided true;
	envmap true;
	texture "texture/Image1";
}
subshader "millenium_body_Material1" "StandardMesh/Default"
{
	lighting false;
	lightingSpecular false;
	materialDiffuse 1 1 1;
	transparent true;
	twosided true;
	texture "texture/Image1";
}
